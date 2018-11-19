FROM nginx
COPY /dist/* /var/dist/
COPY nginx.conf /tmp/nginx.template

CMD /bin/bash -c "envsubst '\$RC_SERVER_HOST' < /tmp/nginx.template > /etc/nginx/nginx.conf && exec nginx -g 'daemon off;'"

