#!/bin/bash
docker run \
	-it \
	--user 1000 \
	--mount type=bind,src=./,dst=/app \
	-w /app \
	mcr.microsoft.com/playwright:v1.59.1-noble \
	npx playwright test
