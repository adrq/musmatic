FROM ubuntu:22.04
RUN apt update && apt install -y \
    apache2 \ 
RUN a2enmod rewrite
COPY apache-config.conf /etc/apache2/sites-available/000-default.conf
RUN service apache2 start
EXPOSE 80
ENTRYPOINT service apache2 start && bash
