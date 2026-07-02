import { test, expect, Page } from '@playwright/test';
import { Term2Api } from '../support/fixtures';

const BASE_URL = process.env.TERM2_BASE_URL || 'http://127.0.0.1:3000';

async function waitForTerminalText(page: Page, needle: string, timeout = 15000): Promise<void> {
  await page.waitForFunction(
    (text) => {
      const log = document.getElementById('term2-e2e-log');
      return log !== null && log.textContent!.includes(text);
    },
    needle,
    { timeout }
  );
}

async function createSession(page: Page, name: string, profile: string): Promise<string> {
  await page.goto('/');
  await expect(page.locator('h1')).toContainText('Term2');

  await page.locator('#session-name').fill(name);
  await page.locator('#profile-select').selectOption(profile);
  await page.locator('#create-form button[type="submit"]').click();

  await page.locator('#sessions-table tbody tr').first().waitFor({ timeout: 5000 });
  const link = page.locator('#sessions-table tbody tr').first().locator('a[href^="/terminal.html"]');
  const href = await link.getAttribute('href');
  const id = new URLSearchParams(href!.split('?')[1]).get('id')!;
  return id;
}

test.describe('warp-style ux', () => {
  test.beforeEach(async () => {
    const api = await Term2Api.create(BASE_URL);
    await api.deleteAllSessions();
    await api.dispose();
  });

  test.afterEach(async () => {
    const api = await Term2Api.create(BASE_URL);
    await api.deleteAllSessions();
    await api.dispose();
  });

  test('portal renders user pill and built-in profiles', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('#user-pill')).toContainText('dev');
    await expect(page.locator('#profiles-list')).toContainText('bash');
    await expect(page.locator('#profiles-list')).toContainText('zsh');
    await expect(page.locator('#profiles-list')).toContainText('nushell');
    await expect(page.locator('#profiles-list')).toContainText('ghr');
  });

  test('creating a session adds it to the session list', async ({ page }) => {
    const id = await createSession(page, `warp-ux-${Date.now()}`, 'bash');
    expect(id).toMatch(/^term2-dev-/);
    await expect(page.locator('#sessions-table')).toContainText('bash');
  });

  test('terminal page connects to a session', async ({ page }) => {
    const id = await createSession(page, `warp-conn-${Date.now()}`, 'bash');
    await page.goto(`/terminal.html?id=${encodeURIComponent(id)}`);
    await waitForTerminalText(page, '[connected]');
  });

  test('terminal receives command output', async ({ page }) => {
    const id = await createSession(page, `warp-output-${Date.now()}`, 'bash');
    await page.goto(`/terminal.html?id=${encodeURIComponent(id)}`);
    await waitForTerminalText(page, '[connected]');
    await page.click('#terminal');
    await page.keyboard.type('echo warp-block-output');
    await page.keyboard.press('Enter');
    await waitForTerminalText(page, 'warp-block-output');
  });

  // Future Warp-like UI features documented as fixme tests.

  test.fixme('command palette opens with keyboard shortcut', async ({ page }) => {
    // CMD-P (macOS) or CTRL-SHIFT-P (Linux/Windows) should open a global
    // command palette with workflows, prompts, notebooks, and actions.
    await page.goto('/');
    await page.keyboard.press('Control+Shift+p');
    await expect(page.locator('[data-testid="command-palette"]')).toBeVisible();
  });

  test.fixme('blocks separate each command and output', async ({ page }) => {
    // Each executed command should appear as a discrete block in the UI.
    const id = await createSession(page, `warp-blocks-${Date.now()}`, 'bash');
    await page.goto(`/terminal.html?id=${encodeURIComponent(id)}`);
    await waitForTerminalText(page, '[connected]');
    await page.click('#terminal');
    await page.keyboard.type('echo first');
    await page.keyboard.press('Enter');
    await page.keyboard.type('echo second');
    await page.keyboard.press('Enter');
    const blocks = page.locator('[data-testid="terminal-block"]');
    await expect(blocks).toHaveCount(2);
  });

  test.fixme('input editor supports multi-line shift-enter', async ({ page }) => {
    // Shift-Enter should insert a newline in the input editor without
    // submitting the command.
    const id = await createSession(page, `warp-input-${Date.now()}`, 'bash');
    await page.goto(`/terminal.html?id=${encodeURIComponent(id)}`);
    await waitForTerminalText(page, '[connected]');
    await page.click('#terminal');
    await page.keyboard.type('for x in a b c;');
    await page.keyboard.press('Shift+Enter');
    await page.keyboard.type('do echo $x; done');
    await expect(page.locator('[data-testid="input-editor"]')).toContainText('for x in a b c;\ndo echo $x; done');
  });

  test.fixme('theme picker changes terminal colors', async ({ page }) => {
    // CTRL-CMD-T should open a theme picker and switch the terminal theme.
    const id = await createSession(page, `warp-theme-${Date.now()}`, 'bash');
    await page.goto(`/terminal.html?id=${encodeURIComponent(id)}`);
    await page.keyboard.press('Control+Alt+t');
    await expect(page.locator('[data-testid="theme-picker"]')).toBeVisible();
  });

  test.fixme('workflows open from command palette', async ({ page }) => {
    // `workflows:` in the palette should list saved workflows.
    await page.goto('/');
    await page.keyboard.press('Control+Shift+r');
    await expect(page.locator('[data-testid="workflows-panel"]')).toBeVisible();
  });

  test.fixme('block can be bookmarked and re-input', async ({ page }) => {
    // Bookmarking a block adds it to a bookmark list; re-input pastes the
    // command back into the input editor.
    const id = await createSession(page, `warp-bookmark-${Date.now()}`, 'bash');
    await page.goto(`/terminal.html?id=${encodeURIComponent(id)}`);
    await waitForTerminalText(page, '[connected]');
    await page.click('#terminal');
    await page.keyboard.type('git status');
    await page.keyboard.press('Enter');
    await page.click('[data-testid="block-menu-button"]');
    await page.click('[data-testid="bookmark-block"]');
    await expect(page.locator('[data-testid="bookmarks-list"]')).toContainText('git status');
  });

  test.fixme('notification mailbox shows agent status', async ({ page }) => {
    // The notification bell icon should open a mailbox with agent Complete,
    // Request, and Error notifications.
    await page.goto('/');
    await page.click('[data-testid="notification-bell"]');
    await expect(page.locator('[data-testid="notification-mailbox"]')).toBeVisible();
  });

  test.fixme('session navigation palette lists running sessions', async ({ page }) => {
    // A session navigation palette should list sessions by prompt, running
    // command, and status.
    await createSession(page, `warp-nav-${Date.now()}`, 'bash');
    await page.keyboard.press('Control+Tab');
    await expect(page.locator('[data-testid="session-navigator"]')).toBeVisible();
  });
});
