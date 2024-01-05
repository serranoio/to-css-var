package main

import (
	"encoding/json"
	"io"
	"net/http"
	"os"
	"os/exec"
	"time"

	"github.com/gin-contrib/cors"
	"github.com/gin-gonic/gin"
)

type Message struct {
	Message string `json:"message"`
}

func main() {
	r := gin.Default()
	r.Use(cors.New(cors.Config{
		AllowAllOrigins:  true,
		AllowMethods:     []string{"PUT", "GET", "PATCH", "POST"},
		AllowHeaders:     []string{"Origin"},
		AllowCredentials: true,
		AllowFiles:       true,
		MaxAge:           12 * time.Hour,
	}))

	r.POST("/convert", func(c *gin.Context) {
		body, _ := io.ReadAll(c.Request.Body)

		message := Message{}
		json.Unmarshal(body, &message)

		os.WriteFile("data.txt", []byte(message.Message), 0755)
		cmd := exec.Command("./to-css-var", "data.txt")
		output, _ := cmd.Output()

		c.JSON(http.StatusOK, gin.H{
			"message": string(output),
		})
	})

	r.Run()
}
