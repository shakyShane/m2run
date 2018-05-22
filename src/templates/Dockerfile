FROM wearejh/m2-php-70:travis-2

# Magento
COPY composer.json composer.lock auth.json ./
COPY .docker/composer-cache .docker/composer-cache

RUN chsh -s /bin/bash www-data \
    && chown -R www-data:www-data ./

RUN su - www-data -c "COMPOSER_CACHE_DIR=.docker/composer-cache composer install --no-interaction --prefer-dist -o"

# Frontend
RUN mkdir -p app/etc
COPY app/etc app/etc
COPY .data-migration .data-migration

RUN find . -user root | xargs chown www-data:www-data && chmod +x bin/magento

VOLUME ["/var/www/app/etc"]
VOLUME ["/var/www/pub"]
VOLUME ["/var/www/setup"]
VOLUME ["/var/www/var"]

ENTRYPOINT ["/usr/local/bin/docker-configure"]
