# Usa una imagen base de Go
FROM golang:1.21.6-alpine

# Establece el directorio de trabajo en /app
WORKDIR /app

# Copia los archivos necesarios para construir la aplicación
COPY go.mod go.sum ./
RUN go mod download

# Copia el resto de la aplicación al directorio de trabajo
COPY . .

# Compila la aplicación
RUN go build main.go

# Expone el puerto 3000
EXPOSE 3001

# Comando para ejecutar la aplicación
CMD ["./main"]