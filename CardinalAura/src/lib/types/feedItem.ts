export interface FeedItem {
  title: string;
  link: string;
  description: string;
  author: null;
  categories: any[];
  comments: null;
  enclosure: Enclosure;
  guid: GUID;
  pub_date: string;
  source: null;
  content: null;
  extensions: Extensions;
  dublin_core_ext: { [key: string]: string[] };
}

export interface Enclosure {
  url: string;
  length: string;
  mime_type: string;
}

export interface Extensions {}

export interface GUID {
  value: string;
  permalink: boolean;
}
