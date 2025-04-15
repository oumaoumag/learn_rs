#!/bin/bash

# push-all.sh - Script to push changes to both origin and upstream repositories
# Created by Augment Agent

# Set colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}Pushing to multiple Git remotes...${NC}"

# Get current branch
CURRENT_BRANCH=$(git symbolic-ref --short HEAD)
if [ $? -ne 0 ]; then
    echo -e "${RED}Error: Failed to determine current branch.${NC}"
    exit 1
fi

echo -e "Current branch: ${GREEN}$CURRENT_BRANCH${NC}"

# Push to origin
echo -e "\n${YELLOW}Pushing to origin...${NC}"
git push origin $CURRENT_BRANCH
if [ $? -ne 0 ]; then
    echo -e "${RED}Error: Failed to push to origin.${NC}"
    echo -e "${YELLOW}You may need to run: git push -u origin $CURRENT_BRANCH${NC}"
    exit 1
fi
echo -e "${GREEN}Successfully pushed to origin.${NC}"

# Push to upstream
echo -e "\n${YELLOW}Pushing to upstream...${NC}"
git push upstream $CURRENT_BRANCH
if [ $? -ne 0 ]; then
    echo -e "${RED}Error: Failed to push to upstream.${NC}"
    echo -e "${YELLOW}You may need to run: git push -u upstream $CURRENT_BRANCH${NC}"
    echo -e "${YELLOW}Or you may not have write access to the upstream repository.${NC}"
    exit 1
fi
echo -e "${GREEN}Successfully pushed to upstream.${NC}"

echo -e "\n${GREEN}All pushes completed successfully!${NC}"
