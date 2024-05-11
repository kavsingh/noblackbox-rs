/** @type {import("path")} */
const path = require("path");

/** @type {import("eslint").ESLint.ConfigData} */
module.exports = {
	root: true,
	extends: [require.resolve("../.eslintrc.cjs")],
	parserOptions: { tsconfigRootDir: __dirname },
	settings: {
		"import/resolver": {
			"eslint-import-resolver-typescript": {
				project: path.join(__dirname, "tsconfig.json"),
			},
		},
	},
	overrides: [
		{
			files: "*.spec.*",
			extends: "plugin:playwright/recommended",
		},
	],
};
