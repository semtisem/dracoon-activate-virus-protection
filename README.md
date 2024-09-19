# DRACOON Activate Virus Protection

## Description

This project aims on activating virus protection for given rooms and all their sub rooms in DRACOON.

## Prerequisites

Authenticated user can only activate virus protection on rooms where he/she is data room admin.

## Usage

1. Register OAuth Client in DRACOON ([instructions](https://cloud.support.dracoon.com/hc/en-us/articles/360018137839-Settings-Apps-incl-OAuth-app-registration))

   <strong>GrandTypes:</strong> authorization_code
   <strong>Callback URL:</strong> <em>$YOUR_DRACOON_DOMAIN</em>/oauth/callback

2. Download GitHub release or build from source
3. Fill config.example.yml
   3.1 Enter your DRACOON domain at <strong>base_url</strong>
   3.2 Set <strong>activate_on_all_rooms</strong> to true if you want to activate Virus Protection on all rooms. Be careful with this setting as file are blocked until scanned. This might take a while when there are a huge amount of file. You can use room ids instead.
   3.2 Enter room ids in the array of <strong>activate_virus_protection</strong> if you want to activate on certain rooms.
   3.3 Rename file to <strong>config.yml</strong>. This file need to be next to the scripts binary.
4. Run binary, authenticate with given link and past back authentication code to terminal.

## License

This project is licensed under the [MIT License](https://opensource.org/licenses/MIT).
