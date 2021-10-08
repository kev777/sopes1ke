package bd

import (
	"database/sql"
	_ "github.com/go-sql-driver/mysql"
)

//getDB -> obtiene variables para la conexion
func GetDB() (*sql.DB, error) {
	return sql.Open("mysql", ConnectionString)
}
