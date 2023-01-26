import axios from 'axios'

declare let process: {
  env: {
    JEST_ENV: string,
    TEST_TENANT_ID_APNS: string,
  }
}

const BASE_URLS = new Map<string, string>([
  ['prod', 'https://history.walletconnect.com'],
  ['staging', 'https://staging.history.walletconnect.com'],
  ['dev', 'https://dev.history.walletconnect.com'],
  ['local', 'http://localhost:3002'],
])

const TEST_TENANT = process.env.TEST_TENANT_ID_APNS

const BASE_URL = BASE_URLS.get(process.env.JEST_ENV)

describe('Gilgamesh', () => {
  describe('Health', () => {
    const url = `${BASE_URL}/health`

    it('is healthy', async () => {
      const { status } = await axios.get(`${url}`)

      expect(status).toBe(200)
    })
  })
  describe('Messages', () => {
    const url = `${BASE_URL}/messages`

    it('can read/write messages', async () => {
      let resp = await axios.get(`${url}`)

      expect(resp.status).toBe(200)

      resp = await axios.post(`${url}`, {})

      expect(resp.status).toBe(200)
    })
  })
})
