import { Agent9527Options } from "./agent9527Options";
import { Agent9527Exec } from "./exec";
import { Thread } from "./thread";
import { ThreadOptions } from "./threadOptions";

/**
 * Agent9527 is the main class for interacting with the Agent9527 agent.
 *
 * Use the `startThread()` method to start a new thread or `resumeThread()` to resume a previously started thread.
 */
export class Agent9527 {
  private exec: Agent9527Exec;
  private options: Agent9527Options;

  constructor(options: Agent9527Options = {}) {
    const { agent9527PathOverride, env, config } = options;
    this.exec = new Agent9527Exec(agent9527PathOverride, env, config);
    this.options = options;
  }

  /**
   * Starts a new conversation with an agent.
   * @returns A new thread instance.
   */
  startThread(options: ThreadOptions = {}): Thread {
    return new Thread(this.exec, this.options, options);
  }

  /**
   * Resumes a conversation with an agent based on the thread id.
   * Threads are persisted in ~/.agent9527/sessions.
   *
   * @param id The id of the thread to resume.
   * @returns A new thread instance.
   */
  resumeThread(id: string, options: ThreadOptions = {}): Thread {
    return new Thread(this.exec, this.options, options, id);
  }
}
