package src

type Tweet struct {
	Id    int64
    Nombre  string `json:"nombre"`
    Comentario string `json:"comentario"`
    Fecha  string `json:"fecha"`
    Hashtags []string `json:"hashtags"`
    Upvotes int64   `json:"upvotes"`
    Downvotes int64 `json:"downvotes"`
    Api string  
}

//mongoDB
type Tweet1 struct {
    Nombre  string 
    Comentario string 
    Fecha  string 
    Hashtags string 
    Upvotes int64   
    Downvotes int64 
    Api string  
}