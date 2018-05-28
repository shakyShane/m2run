FROM shakyshane/m2run-contrib-base:travis-3

#RUN find . -user root | xargs chown www-data:www-data

VOLUME ["/var/www/app/etc"]
VOLUME ["/var/www/pub"]
VOLUME ["/var/www/setup"]
VOLUME ["/var/www/var"]
