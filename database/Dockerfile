# Use the official PostgreSQL image as base
FROM postgres:17

# Environment variables for database configuration
ENV POSTGRES_DB=mydb
ENV POSTGRES_USER=myuser
ENV POSTGRES_PASSWORD=mypassword

# Create directory for custom initialization scripts
RUN mkdir -p /docker-entrypoint-initdb.d

# Copy initialization script
COPY ./init.sql /docker-entrypoint-initdb.d/

# Create a directory for persistent storage
RUN mkdir -p /var/lib/postgresql/data

# Set proper permissions
RUN chown -R postgres:postgres /var/lib/postgresql/data

# Set the data directory as a volume
VOLUME /var/lib/postgresql/data

# Expose the PostgreSQL port
EXPOSE 5432

# The default command will start PostgreSQL
CMD ["postgres"]