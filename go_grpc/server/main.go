package main

import (
	"context"
	"fmt"
	"log"
	"net"

	pb "server.com/m/server"

	"github.com/IBM/sarama"
	"google.golang.org/grpc"
)

type server struct {
	pb.UnimplementedGetInfoServer
}

const (
	port          = ":3001"
	kafkaBroker   = "go-grpc-server:9092" // Cambia esto si Kafka está en otro lugar
	kafkaTopic    = "test_topic"          // Cambia esto al nombre de tu topic Kafka
	kafkaConsumer = "consumer_group"      // Cambia esto al nombre de tu grupo de consumidores
)

type Data struct {
	Album  string
	Year   string
	Artist string
	Ranked string
}

func (s *server) ReturnInfo(ctx context.Context, in *pb.RequestId) (*pb.ReplyInfo, error) {
	fmt.Println("Recibí de cliente: ", in.GetArtist())
	data := Data{
		Year:   in.GetYear(),
		Album:  in.GetAlbum(),
		Artist: in.GetArtist(),
		Ranked: in.GetRanked(),
	}
	fmt.Println(data)

	// Realizar suscripción
	go subscribeToKafka()

	// Realizar publicación
	go publishToKafka(&data)

	return &pb.ReplyInfo{Info: "Hola cliente, recibí el álbum"}, nil
}

func subscribeToKafka() {
	config := sarama.NewConfig()
	config.Consumer.Return.Errors = true

	consumer, err := sarama.NewConsumer([]string{kafkaBroker}, config)
	if err != nil {
		log.Fatalln("Error al crear el consumidor:", err)
	}
	defer func() {
		if err := consumer.Close(); err != nil {
			log.Fatalln("Error al cerrar el consumidor:", err)
		}
	}()

	partitionConsumer, err := consumer.ConsumePartition(kafkaTopic, 0, sarama.OffsetNewest)
	if err != nil {
		log.Fatalln("Error al crear el consumidor de particiones:", err)
	}
	defer func() {
		if err := partitionConsumer.Close(); err != nil {
			log.Fatalln("Error al cerrar el consumidor de particiones:", err)
		}
	}()

	for {
		select {
		case msg := <-partitionConsumer.Messages():
			fmt.Println("Mensaje recibido de Kafka:", string(msg.Value))
		case err := <-partitionConsumer.Errors():
			log.Println("Error al consumir mensaje de Kafka:", err)
		}
	}
}

func publishToKafka(data *Data) {
	config := sarama.NewConfig()
	config.Producer.Return.Successes = true

	producer, err := sarama.NewSyncProducer([]string{kafkaBroker}, config)
	if err != nil {
		log.Fatalln("Error al crear el productor de Kafka:", err)
	}
	defer func() {
		if err := producer.Close(); err != nil {
			log.Fatalln("Error al cerrar el productor de Kafka:", err)
		}
	}()

	message := &sarama.ProducerMessage{
		Topic: kafkaTopic,
		Value: sarama.StringEncoder(fmt.Sprintf("Album: %s, Year: %s, Artist: %s, Ranked: %s", data.Album, data.Year, data.Artist, data.Ranked)),
	}

	partition, offset, err := producer.SendMessage(message)
	if err != nil {
		log.Fatalln("Error al enviar mensaje a Kafka:", err)
	}

	fmt.Printf("Mensaje enviado a la partición %d con offset %d\n", partition, offset)
}

func main() {
	listen, err := net.Listen("tcp", port)
	if err != nil {
		log.Fatalln(err)
	}
	s := grpc.NewServer()
	pb.RegisterGetInfoServer(s, &server{})

	if err := s.Serve(listen); err != nil {
		log.Fatalln(err)
	}
}
