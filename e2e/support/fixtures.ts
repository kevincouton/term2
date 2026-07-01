import { request, APIRequestContext } from '@playwright/test';

export interface SessionInfo {
  id: string;
  name: string;
  profile: string;
  created_at: number;
  attached: boolean;
}

export class Term2Api {
  constructor(private ctx: APIRequestContext) {}

  static async create(baseURL: string): Promise<Term2Api> {
    const ctx = await request.newContext({ baseURL });
    return new Term2Api(ctx);
  }

  async dispose() {
    await this.ctx.dispose();
  }

  async listSessions(): Promise<SessionInfo[]> {
    const res = await this.ctx.get('/api/v1/sessions');
    if (!res.ok()) throw new Error(`listSessions: ${res.status()}`);
    const data = await res.json();
    return data as SessionInfo[];
  }

  async createSession(name: string, profile: string): Promise<SessionInfo> {
    const res = await this.ctx.post('/api/v1/sessions', {
      data: { name, profile },
    });
    if (!res.ok()) throw new Error(`createSession: ${res.status()} ${await res.text()}`);
    const data = await res.json();
    return (data as { session: SessionInfo }).session;
  }

  async deleteSession(id: string) {
    const res = await this.ctx.delete(`/api/v1/sessions/${encodeURIComponent(id)}`);
    if (!res.ok() && res.status() !== 404) {
      throw new Error(`deleteSession: ${res.status()}`);
    }
  }

  async deleteAllSessions() {
    const sessions = await this.listSessions();
    await Promise.all(sessions.map((s) => this.deleteSession(s.id)));
  }
}
