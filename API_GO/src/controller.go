package src

import (
	"main/bd"
	"strings"
	"fmt"
	
	"context"
	"time"

	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"

)

var (
	database   string
	collection string
)

const (
	mongoDBConnectionStringEnvVarName = "mongodb://mongosopes1p1:ij8uDqTbzpPyFot5sB7ozhiOpOw5dZZo2sV3fAscHvLHkBAvRgK9YpBrykNSijGbFATiGIJzDrgfVaKnliOZMQ==@mongosopes1p1.mongo.cosmos.azure.com:10255/?ssl=true&retrywrites=false&replicaSet=globaldb&maxIdleTimeMS=120000&appName=@mongosopes1p1@"
	mongoDBDatabaseEnvVarName         = "local"
	mongoDBCollectionEnvVarName       = "tweet"
)

func createTweet(tweet Tweet) error {
	base, err := bd.GetDB()
	if err != nil {
		return err
	}

	entrada := tweet.Fecha
    array := strings.Split(entrada, "/")
	fechaNueva := array[2] + "-" + array[1] + "-" + array[0]
	hash:= strings.Join(tweet.Hashtags,",")

	_, err = base.Exec("INSERT INTO tweet (nombre,comentario,fecha,hashtags,upvotes,downvotes,api) VALUES (?,?,?,?,?,?,?)",tweet.Nombre, tweet.Comentario, fechaNueva,hash,tweet.Upvotes,tweet.Downvotes,"go")

	c := connect()
	ctx := context.Background()
	defer c.Disconnect(ctx)
	collection := c.Database(database).Collection(collection)
	r, err := collection.InsertOne(ctx, Tweet1{Nombre:tweet.Nombre,Comentario:tweet.Comentario,Fecha:fechaNueva,Hashtags:hash,Upvotes:tweet.Upvotes,Downvotes:tweet.Downvotes,Api:"go"})
	if err != nil {
		fmt.Println("Fallo al insertar",err)
	}else{
		fmt.Println("Un nuevo tweet", r.InsertedID)
	}
	

	return err
}

func connect() *mongo.Client {
	mongoDBConnectionString := mongoDBConnectionStringEnvVarName
	if mongoDBConnectionString == "" {
		fmt.Println("No hay string de conexion")
	}

	database =mongoDBDatabaseEnvVarName
	if database == "" {
		fmt.Println("falta variable")
	}

	collection = mongoDBCollectionEnvVarName
	if collection == "" {
		fmt.Println("falta variable")
	}

	ctx, cancel := context.WithTimeout(context.Background(), time.Second*10)
	defer cancel()

	clientOptions := options.Client().ApplyURI(mongoDBConnectionString).SetDirect(true)
	c, err := mongo.NewClient(clientOptions)

	err = c.Connect(ctx)

	if err != nil {
		fmt.Println("No se puede realizar la conexion",err)
	}
	/*err = c.Ping(ctx, nil)
	if err != nil {
		fmt.Println("falta variable",err)
	}*/
	return c
}



