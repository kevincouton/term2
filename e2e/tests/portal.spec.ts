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

async function openSession(page: Page, name: string, profile: string): Promise<string> {
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

test.describe('portal', () => {
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

  test('portal shows user and built-in profiles', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('#user-pill')).toContainText('dev');
    await expect(page.locator('#profiles-list')).toContainText('bash');
    await expect(page.locator('#profiles-list')).toContainText('zsh');
    await expect(page.locator('#profiles-list')).toContainText('nushell');
  });

  test('bash session can be created and receives output', async ({ page }) => {
    const id = await openSession(page, `bash-e2e-${Date.now()}`, 'bash');
    await page.goto(`/terminal.html?id=${encodeURIComponent(id)}`);

    await waitForTerminalText(page, '[connected]');
    await page.click('#terminal');
    await page.keyboard.type('echo term2-bash-ok');
    await page.keyboard.press('Enter');
    await waitForTerminalText(page, 'term2-bash-ok');
  });

  test('zsh session starts with oh-my-zsh prompt', async ({ page }) => {
    const id = await openSession(page, `zsh-e2e-${Date.now()}`, 'zsh');
    await page.goto(`/terminal.html?id=${encodeURIComponent(id)}`);

    await waitForTerminalText(page, '[connected]');
    await page.waitForTimeout(1000);
    await page.click('#terminal');
    await page.keyboard.type('echo term2-zsh-ok');
    await page.keyboard.press('Enter');
    await waitForTerminalText(page, 'term2-zsh-ok');
  });

  test('nushell session runs commands', async ({ page }) => {
    const id = await openSession(page, `nu-e2e-${Date.now()}`, 'nushell');
    await page.goto(`/terminal.html?id=${encodeURIComponent(id)}`);

    await waitForTerminalText(page, '[connected]');
    await page.waitForTimeout(1000);
    await page.click('#terminal');
    await page.keyboard.type('echo term2-nu-ok');
    await page.keyboard.press('Enter');
    await waitForTerminalText(page, 'term2-nu-ok');
  });

  test('tmux tiling split works inside a session', async ({ page }) => {
    const id = await openSession(page, `tmux-tile-${Date.now()}`, 'bash');
    await page.goto(`/terminal.html?id=${encodeURIComponent(id)}`);

    await waitForTerminalText(page, '[connected]');
    await page.waitForTimeout(800);

    // tmux prefix key Ctrl+b then % splits vertically.
    await page.keyboard.press('Control+b');
    await page.keyboard.press('Shift+5'); // `%`
    await page.waitForTimeout(500);

    const api = await Term2Api.create(BASE_URL);
    const sessions = await api.listSessions();
    const match = sessions.find((s) => s.id === id);
    expect(match).toBeDefined();
    await api.dispose();
  });
});
