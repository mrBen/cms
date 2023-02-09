package main

import (
	"fmt"
	"io/fs"
	"log"
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
	if match == nil {
		return nil
	}
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
	// 	log.Fatal("please specify a folder")
	// }

	// folder := os.Args[1]
	folder, _ := filepath.Abs(`~/Downloads/Torrents`)

	videos := listVideos(folder)

	episodes := make(map[string][]Episode)
	for _, video := range videos {
		log.Println(video)
		episode := NewEpisode(video)
		if episode == nil {
			log.Fatal("not a series episode")
		}
		episodes["hello"] = append(episodes["hello"], *episode)
	}

	fmt.Println(episodes)
}

func listVideos(folder string) []string {
	videos := make([]string, 0)
	err := filepath.WalkDir(folder, func(path string, info fs.DirEntry, err error) error {
		if err != nil {
			return nil
		}
		videos = append(videos, path)
		return nil
	})
	if err != nil {
		return nil
	}
	return videos
}
