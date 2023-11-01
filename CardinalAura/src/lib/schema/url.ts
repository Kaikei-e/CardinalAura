import { z } from "zod";

export const urlValidationSchema = z.string().url();
