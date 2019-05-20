FROM wearejh/php:7.1

WORKDIR /var/www

COPY composer.json composer.lock auth.json ./
COPY .docker/composer-cache .docker/composer-cache
RUN mkdir -p app/etc vendor var generation lib pub
COPY app app
COPY patches patches

RUN chsh -s /bin/bash www-data \
    && chown -R www-data:www-data ./

#RUN su - www-data -c "COMPOSER_CACHE_DIR=.docker/composer-cache composer install --no-interaction --prefer-dist -o"

RUN chown -R www-data:www-data var generation pub
#RUN chmod +x bin/magento

#RUN su - www-data -c "COMPOSER_CACHE_DIR=.docker/composer-cache composer dumpautoload --no-interaction -o"

ENV COMPOSER_CACHE_DIR=.docker/composer-cache

VOLUME ["/var/www/pub"]
VOLUME ["/var/www/var"]
VOLUME ["/var/www/vendor"]
VOLUME ["/var/www/lib"]

ENTRYPOINT ["/usr/local/bin/docker-configure"]
CMD ["php-fpm"]
