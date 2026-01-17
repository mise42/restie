import { z } from 'zod';

export const breakSettingsSchema = z.object({
  microbreak_interval_minutes: z.number()
    .int()
    .min(1, "Microbreak interval must be at least 1 minute")
    .max(60, "Microbreak interval cannot exceed 60 minutes"),

  microbreak_duration_seconds: z.number()
    .int()
    .min(5, "Microbreak duration must be at least 5 seconds")
    .max(300, "Microbreak duration cannot exceed 5 minutes"),

  longbreak_interval_microbreaks: z.number()
    .int()
    .min(1, "Long break interval must be at least 1 microbreak")
    .max(10, "Long break interval cannot exceed 10 microbreaks"),

  longbreak_duration_minutes: z.number()
    .int()
    .min(1, "Long break duration must be at least 1 minute")
    .max(60, "Long break duration cannot exceed 60 minutes"),

  fullscreen_breaks: z.boolean(),
});

export type BreakSettings = z.infer<typeof breakSettingsSchema>;
