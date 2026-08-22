package main

import (
	"fmt"
	"strings"
	"time"

	"github.com/inancgumus/screen"
	"moul.io/banner"
)

func main() {
	screen.Clear()
	screen.MoveTopLeft()
	fmt.Println(banner.Inline("tescii"))
	fmt.Println("Text to Ascii Art in Go")
	time.Sleep(2 * time.Second)
	var text string
	fmt.Println("Text:")
	fmt.Scanln(&text)
	fmt.Println(banner.Inline(strings.ToLower(text)))
}
