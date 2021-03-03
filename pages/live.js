import Head from "next/head";
import { Visual } from "../components/Visual";
import { LiveFooter } from "../components/LiveFooter";
import { SubTitle } from "../components/SubTitle";
import { Title } from "../components/Title";
import { SourceCodeFooter } from "../components/SourceCodeFooter";
import { Container } from "../components/Container";
import { GlobalLive } from "../components/Global";
import { Main } from "../components/Main";
import { Header } from "../components/Header";
import { getToday } from "../day";

export default function Home() {
  const Day = getToday();
  return (
    <GlobalLive>
      <Container>
        <Head>
          <title>shaderday.com</title>
          <link rel="icon" href="/favicon.ico" />
        </Head>
        <Main>
          <Header>
            <Title text="shaderday.com" />
          </Header>
          <Visual Day={Day} />
        </Main>
      </Container>
    </GlobalLive>
  );
}