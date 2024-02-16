// import { OAuth2Client } from "google-auth-library";
// import { readTextFile, writeTextFile } from "@tauri-apps/api/fs";
// import { appLocalDataDir } from "@tauri-apps/api/path";
// const path = require("path");
// const { authenticate } = require("@google-cloud/local-auth");
// const { google } = require("googleapis");

// // If modifying these scopes, delete token.json.
// const SCOPES = ["https://www.googleapis.com/auth/documents.readonly"];
// // The file token.json stores the user's access and refresh tokens, and is
// // created automatically when the authorization flow completes for the first
// // time.
// const TOKEN_PATH = path.join(appLocalDataDir(), "token.json");
// const CREDENTIALS_PATH = path.join(appLocalDataDir(), "credentials.json");

// /**
//  * Reads previously authorized credentials from the save file.
//  *
//  * @return {Promise<OAuth2Client|null>}
//  */
// async function loadSavedCredentialsIfExist() {
//   try {
//     const content = await readTextFile(TOKEN_PATH);
//     const credentials = JSON.parse(content);
//     return google.auth.fromJSON(credentials);
//   } catch (err) {
//     return null;
//   }
// }

// /**
//  * Serializes credentials to a file comptible with GoogleAUth.fromJSON.
//  *
//  * @param {OAuth2Client} client
//  * @return {Promise<void>}
//  */
// async function saveCredentials(client: OAuth2Client) {
//   const content = await readTextFile(CREDENTIALS_PATH);
//   const keys = JSON.parse(content);
//   const key = keys.installed || keys.web;
//   const payload = JSON.stringify({
//     type: "authorized_user",
//     client_id: key.client_id,
//     client_secret: key.client_secret,
//     refresh_token: client.credentials.refresh_token,
//   });
//   await writeTextFile(TOKEN_PATH, payload);
// }

// /**
//  * Load or request or authorization to call APIs.
//  *
//  */
// async function authorize() {
//   let client = await loadSavedCredentialsIfExist();
//   if (client) {
//     return client;
//   }
//   client = await authenticate({
//     scopes: SCOPES,
//     keyfilePath: CREDENTIALS_PATH,
//   });
//   if (client.credentials) {
//     await saveCredentials(client);
//   }
//   return client;
// }

// /**
//  * Prints the title of a sample doc:
//  * https://docs.google.com/document/d/195j9eDD3ccgjQRttHhJPymLJUCOUjs-jmwTrekvdjFE/edit
//  * @param {google.auth.OAuth2} auth The authenticated Google OAuth 2.0 client.
//  */
// async function printDocTitle(auth: OAuth2Client) {
//   const docs = google.docs({ version: "v1", auth });
//   const res = await docs.documents.get({
//     documentId: "195j9eDD3ccgjQRttHhJPymLJUCOUjs-jmwTrekvdjFE",
//   });
//   console.log(`The title of the document is: ${res.data.title}`);
// }

// authorize().then(printDocTitle).catch(console.error);

// export async function printYoutubeVideo(videoId: string) {
//   const auth = await authorize();
//   const youtube = google.youtube({ version: "v3", auth });
//   const res = await youtube.videos.list({
//     id: videoId,
//     part: "snippet",
//   });
//   console.log(`The title of the video is: ${res.data.items[0].snippet.title}`);
// }
