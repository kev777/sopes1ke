from flask import Flask, app, request
from waitress import serve
import json
import logging
from flask_mysqldb import MySQL
from pymongo import MongoClient

app = Flask(__name__)

app.config['MYSQL_HOST'] = '35.188.131.97'
app.config['MYSQL_USER'] = 'root'
app.config['MYSQL_PASSWORD'] = 'grupo30'
app.config['MYSQL_DB'] = 'Proyecto1'

mysql = MySQL(app)

uri = f'mongodb://mongosopes1p1:ij8uDqTbzpPyFot5sB7ozhiOpOw5dZZo2sV3fAscHvLHkBAvRgK9YpBrykNSijGbFATiGIJzDrgfVaKnliOZMQ==@mongosopes1p1.mongo.cosmos.azure.com:10255/?ssl=true&retrywrites=false&replicaSet=globaldb&maxIdleTimeMS=120000&appName=@mongosopes1p1@'
mongo_client = MongoClient(uri)
my_db = mongo_client["local"]
my_col = my_db["tweet"]

@app.route('/')
def hello_world():
    return 'Api en PYTHON'

@app.route('/Carga', methods = ['POST'])
def iniciarCarga():
    print('Petición POST realizada')
    if request.method == 'POST':
        content = request.get_json()
        nombre = content['nombre']
        comentario = content['comentario']
        fecha = content['fecha']
        fecha_array = fecha.split("/")
        fecha_nueva = fecha_array[2]+"-"+fecha_array[1]+"-"+fecha_array[0]
        hashtags = content['hashtags']
        hashtag_string = ','.join(hashtags)
        upvotes = content['upvotes']
        downvotes = content['downvotes']
        #conexion a bd mysql       
        cur = mysql.connection.cursor()
        cur.execute("INSERT INTO tweet(nombre,comentario,fecha,hashtags,upvotes,downvotes,api) VALUES (%s,%s,%s,%s,%s,%s,'python');", 
        (nombre,comentario,fecha_nueva,hashtag_string,upvotes,downvotes))
        mysql.connection.commit()
        cur.close()
        #conexion a cosmos db
        tweet = {}
        tweet['nombre'] = nombre
        tweet['comentario'] = comentario
        tweet['fecha'] = fecha_nueva
        tweet['hashtags'] = hashtag_string
        tweet['upvotes'] = upvotes
        tweet['downvotes'] = downvotes
        tweet['api'] = "python"

        document = my_col.insert_one(tweet)
        print(document.inserted_id)

        return 'success',200


        
if __name__ == '__main__':
    serve(app, host="0.0.0.0", port=3050)