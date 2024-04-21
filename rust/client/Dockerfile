# Etapa de construcción del cliente
FROM rust:1.77.0 as client-build-env
WORKDIR /app/client
ADD . /app/client
RUN rustup target add wasm32-wasi
RUN cargo build --target wasm32-wasi --release


# Etapa final para el contenedor
FROM wasmedge/slim-runtime:0.10.1

# Copia el cliente wasm
WORKDIR /app
COPY --from=client-build-env /app/client/target/wasm32-wasi/release /client

# Exponer el puerto 8000
EXPOSE 8000


# CMD para ejecutar tanto el cliente como el servidor
CMD ["wasmedge", "--dir", "./:/", "/client/client.wasm"]
