package bd

import (
	"fmt"
	//"github.com/joho/godotenv"
	//"os"
)

//var _ = godotenv.Load(".env") // Cargar del archivo llamado ".env"
var (
	ConnectionString = fmt.Sprintf("%s:%s@tcp(%s:%s)/%s",
			"root",
			"grupo30",
			"35.188.131.97",
			"3306",
			"Proyecto1")
	)


