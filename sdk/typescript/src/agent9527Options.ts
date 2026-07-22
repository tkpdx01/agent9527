export type Agent9527ConfigValue = string | number | boolean | Agent9527ConfigValue[] | Agent9527ConfigObject;

export type Agent9527ConfigObject = { [key: string]: Agent9527ConfigValue };

export type Agent9527Options = {
  agent9527PathOverride?: string;
  baseUrl?: string;
  apiKey?: string;
  /**
   * Additional `--config key=value` overrides to pass to the Agent9527 CLI.
   *
   * Provide a JSON object and the SDK will flatten it into dotted paths and
   * serialize values as TOML literals so they are compatible with the CLI's
   * `--config` parsing.
   */
  config?: Agent9527ConfigObject;
  /**
   * Environment variables passed to the Agent9527 CLI process. When provided, the SDK
   * will not inherit variables from `process.env`.
   */
  env?: Record<string, string>;
};
