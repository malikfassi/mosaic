import { test, expect } from '@playwright/test'

test.beforeEach(async ({ page }) => {
  // Mock Keplr wallet
  await page.addInitScript(() => {
    Object.defineProperty(window, 'keplr', {
      value: {
        enable: async () => Promise.resolve(),
        getKey: async () => ({
          bech32Address: 'stars1mock...',
          pubKey: new Uint8Array([1, 2, 3]),
        }),
      },
      writable: true,
      configurable: true,
    })
  })

  await page.goto('http://localhost:3000')
})

test('should render the pixel canvas', async ({ page }) => {
  // Check if the canvas is rendered
  const canvas = page.locator('canvas')
  await expect(canvas).toBeVisible()
})

test('should connect to wallet', async ({ page }) => {
  // Click the connect button
  const connectButton = page.getByRole('button', { name: /connect/i })
  await connectButton.click()

  // Check if the address is displayed
  const address = page.getByText(/stars1mock.../)
  await expect(address).toBeVisible()
}) 