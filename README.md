# Project Name: Dracoon Activate Virus Protection

## Description:

This project aims on activating virus protection for given rooms and all their sub rooms in DRACOON.

## Usage:

1. Register OAuth Client in DRACOON ([manuel] (https://cloud.support.dracoon.com/hc/en-us/articles/360018137839-Settings-Apps-incl-OAuth-app-registration))
   1.1 Use Client ID: Q8dTruVvswW5Iyi0QWqiZFKP8gFgFjnZ, Client Secret: nhh27DqlmFEjf5ijVAN0ZoBdhKMDu4lv, GrandTypes: authorization_code, Callback: $YOUR_DRACOON_DOMAIN/oauth/callback
2. Download GitHub release or build from source
3. Fill config.example.yml
   2.1 Enter your DRACOON domain under "base_url"
   2.2 Enter room ids in the array of "activate_virus_protection"
   2.3 Rename file to config.yml. This file need to be next to the scripts binary.

## License:

This project is licensed under the [MIT License](https://opensource.org/licenses/MIT).
