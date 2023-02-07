package main

import (
	"fmt"
	"path/filepath"
	"regexp"
	"strconv"
)

var numbering = regexp.MustCompile(`[Ss](\d+)[Ee](\d+)`)

type Episode struct {
	season   int
	number   int
	filename string
}

func NewEpisode(filename string) *Episode {
	match := numbering.FindStringSubmatch(filepath.Base(filename))
	season, _ := strconv.Atoi(match[1])
	number, _ := strconv.Atoi(match[2])

	return &Episode{
		filename: filename,
		season:   season,
		number:   number,
	}
}

func main() {
	// if len(os.Args) < 2 {
	// 	fmt.Println("please specify a folder")
	// }

	// folder := os.Args[1]
	folder := `~/Downloads/Torrents/The.Peripheral.S01E10.REPACK.720p.WEB.x265-MiNX\[TGx]/`

	episodes := make(map[string][]Episode)
	episodes["hello"] = append(episodes["hello"], *NewEpisode(folder))

	fmt.Println(episodes)
}
