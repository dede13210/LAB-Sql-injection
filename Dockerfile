# Use the official MySQL image from the Docker Hub
FROM mysql:8.0

# Set environment variables for MySQL
ENV MYSQL_DATABASE=clientdb
ENV MYSQL_USER=myuser
ENV MYSQL_PASSWORD=mypassword
ENV MYSQL_ROOT_PASSWORD=rootpassword

# Add a custom SQL script to initialize the database (optional)
COPY ./app/init.sql /docker-entrypoint-initdb.d/

# Expose the default MySQL port
EXPOSE 3306
