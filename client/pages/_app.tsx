import '../styles/globals.scss'
import type { AppProps } from 'next/app'
import { MantineProvider } from '@mantine/core';

function MyApp({ Component, pageProps }: AppProps) {
  return (
    <MantineProvider
      withGlobalStyles
      withNormalizeCSS
      theme={{
        fontFamily: 'Greycliff CF, sans-serif',
        headings: {
          fontFamily: 'Greycliff CF, sans-serif',
        },
        colorScheme: 'dark',
        
      }}
    >
      <Component {...pageProps} />
    </MantineProvider>
  )
}

export default MyApp
