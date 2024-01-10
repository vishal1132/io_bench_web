package main

import (
	"context"
	"fmt"
	"io"
	"log"
	"math/rand"
	"net/http"
	"sync/atomic"
	"time"

	"github.com/redis/go-redis/v9"
)

func main() {
	var counter atomic.Int32
	client := redis.NewClient(&redis.Options{})
	ctx := context.Background()
	client.Set(ctx, "test", "test_data", 0)
	http.HandleFunc("/data", func(w http.ResponseWriter, r *http.Request) {
		io.WriteString(w, fmt.Sprintf("expensive requests %d", counter.Load()))
	})
	http.HandleFunc("/test", func(w http.ResponseWriter, req *http.Request) {
		if rand.Intn(100)+1 < 6 {
			counter.Add(1)
			time.Sleep(3 * time.Second)
		}
		res, err := client.Get(ctx, "test").Result()
		if err != nil {
			log.Println(err)
		}
		io.WriteString(w, res)
	})
	http.ListenAndServe(":8081", nil)
}
