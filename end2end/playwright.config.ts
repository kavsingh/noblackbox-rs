import { devices } from "@playwright/test";

import type { PlaywrightTestConfig } from "@playwright/test";

const CI = !!process.env["CI"];

/**
 * See https://playwright.dev/docs/test-configuration.
 */
export default {
	testDir: "./tests",
	timeout: 30 * 1000,
	expect: { timeout: 5000 },
	fullyParallel: true,
	forbidOnly: CI,
	retries: CI ? 2 : 0,
	workers: CI ? 1 : "50%",
	reporter: "html",
	use: {
		actionTimeout: 0,
		baseURL: "http://localhost:3000",
		trace: "on-first-retry",
	},
	projects: [
		{
			name: "chromium",
			use: { ...devices["Desktop Chrome"] },
		},
		{
			name: "firefox",
			use: { ...devices["Desktop Firefox"] },
		},
		{
			name: "webkit",
			use: { ...devices["Desktop Safari"] },
		},
	],
} satisfies PlaywrightTestConfig;
