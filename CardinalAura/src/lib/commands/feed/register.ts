import { invoke } from "@tauri-apps/api/tauri";
import { z } from "zod";
import { FeedItem } from "../../types/feedItem";

export const registerSingleFeedLink = async (
  url: string,
): Promise<FeedItem | undefined> => {
  const parsedUrl = z.string().url().parse(url);

  console.log("Parsed URL: ", parsedUrl);
  console.log("starting invoke");

  const result = invoke("invoke_register_single_feed_link_command", {
    url: parsedUrl,
  });

  console.debug("Result: ", result);

  return await result
    .then((res) => {
      console.log("Success");
      if (typeof res === "string") {
        const feedItem: FeedItem = JSON.parse(res);
        return feedItem;
      } else {
        throw new Error("res is not a string");
      }
    })
    .catch((err) => {
      console.log("Error");
      return err;
    });
};
