import { invoke } from "@tauri-apps/api/tauri";
import { z } from "zod";
import { SingleFeedLink } from "../../types/feedItem";

export const registerSingleFeedLink = async (
  url: string,
): Promise<SingleFeedLink | undefined> => {
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
        try {
          const singleFeedLink: SingleFeedLink = {
            url: res
          }
          return singleFeedLink;
        } catch (error) {
          console.error("Failed to parse JSON:", res);
          throw error;
        }
      } else {
        throw new Error("res is not a string");
      }
    })
};
