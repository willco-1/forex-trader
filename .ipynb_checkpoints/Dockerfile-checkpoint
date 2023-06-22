FROM python:3.9

LABEL William "wconyea@gmail.com"

ADD . /app

WORKDIR /app

RUN pip install -r /requirements.txt
COPY data/app.d /app.d
RUN apt-get update && apt-get install -y build-essential redis-server sqlite3 \
    curl software-properties-common

# install nodejs and wscat websocket client
RUN curl -sL https://deb.nodesource.com/setup_14.x | bash -

RUN apt-get install -y nodejs && npm install wscat



# install dependencies
RUN pip3 install -r requirements.txt