import "../styles/globals.scss";
import type { AppProps } from "next/app";
import { MantineProvider, ColorSchemeProvider, ColorScheme } from "@mantine/core";
import theme from "open-color";
import { useState } from "react"

function MyApp({ Component, pageProps }: AppProps) {
  const [colorScheme, setColorScheme] = useState<ColorScheme>('dark');
  const toggleColorScheme = (value?: ColorScheme) =>
    setColorScheme(value || (colorScheme === 'dark' ? 'light' : 'dark'));

  return (
    <ColorSchemeProvider colorScheme={colorScheme} toggleColorScheme={toggleColorScheme}>
      <MantineProvider
      withGlobalStyles
      withNormalizeCSS
        theme={{
          colorScheme, 
          fontFamily: "Greycliff CF, sans-serif",
          colors: {
            purple: [
              "#FAF5FF",
              "#E9D8FD",
              "#D6BCFA",
              "#B794F4",
              "#9F7AEA",
              "#805AD5",
              "#6B46C1",
              "#553C9A",
              "#44337A",
              "#322659",
            ],
          },
          headings: {
            fontFamily: "Greycliff CF, sans-serif",
          },
          primaryColor: "purple",
        }}
      >
        <Component {...pageProps} />
      </MantineProvider>
    </ColorSchemeProvider>
  );
}

export default MyApp;
