package server

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

func NewRouter() http.Handler {
	gin.SetMode(gin.ReleaseMode)

	router := gin.New()
	router.Use(gin.Recovery())

	router.GET("/healthz", func(c *gin.Context) {
		c.JSON(http.StatusOK, gin.H{"status": "ok"})
	})

	router.GET("/api/health", func(c *gin.Context) {
		c.JSON(http.StatusOK, gin.H{
			"service": "api",
			"status":  "ok",
		})
	})

	return router
}
