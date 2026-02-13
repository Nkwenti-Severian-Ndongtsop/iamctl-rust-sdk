import React from 'react';
import clsx from 'clsx';
import Link from '@docusaurus/Link';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';
import Heading from '@theme/Heading';

import styles from './index.module.css';

function HomepageHeader() {
  const {siteConfig} = useDocusaurusContext();
  return (
    <header className={styles.hero}>
      <div className={styles.heroInner}>
        <div className={styles.heroLeft}>
          <Heading as="h1" className={styles.heroTitle}>
            iamctl
            <br />
            Rust SDK
          </Heading>
          <p className={styles.heroSubtitle}>
            All the tools you need to build secure, type-safe IAM providers using Rust.
          </p>

          <div className={styles.heroActions}>
            <Link className={clsx('button', styles.heroCta)} to="/docs/getting-started">
              Get started
              <span className={styles.heroCtaArrow} aria-hidden="true">
                →
              </span>
            </Link>
          </div>
        </div>

        <div className={styles.heroRight} aria-hidden="true">
          <div className={styles.logoMarkWrap}>
            <img src="img/logo.png" alt="" className={styles.logoMark} />
          </div>
        </div>
      </div>
    </header>
  );
}

const HowItWorksList = [
  {
    icon: '📦',
    title: 'Define Resources',
    description: (
      <>
        Describe your IAM resources with Rust structs.
        Derive schemas and validation automatically.
      </>
    ),
  },
  {
    icon: '⚙️',
    title: 'Implement Provider Logic',
    description: (
      <>
        Implement the Provider trait: plan, apply, validate, import.
        iamctl handles the JSON-RPC server and state backend.
      </>
    ),
  },
  {
    icon: '🚀',
    title: 'Run with iamctl Engine',
    description: (
      <>
        iamctl spawns your provider as a subprocess and communicates
        via JSON‑RPC. Built‑in locking and observability keep operations safe.
      </>
    ),
  },
];

function HowItWorksStep({icon, title, description}) {
  return (
    <div className={clsx('col col--4', styles.howItWorksStep)}>
      <div className={styles.howItWorksIcon}>{icon}</div>
      <Heading as="h3">{title}</Heading>
      <p>{description}</p>
    </div>
  );
}

const FeatureList = [
  {
    icon: '🚀',
    title: 'Lightning Fast',
    description: (
      <>
        Built with Rust's zero-cost abstractions and compiled to native code. 
        No runtime overhead, just pure performance.
      </>
    ),
  },
  {
    icon: '🛡️',
    title: 'Type Safe by Default',
    description: (
      <>
        Leverage Rust's ownership model and type system. Compile-time guarantees 
        prevent entire classes of runtime errors.
      </>
    ),
  },
  {
    icon: '🔧',
    title: 'Batteries Included',
    description: (
      <>
        JSON-RPC server, state management, schema validation, and logging 
        all built-in. Focus on your logic, not boilerplate.
      </>
    ),
  },
];

function Feature({icon, title, description}) {
  return (
    <div className={clsx('col col--4', styles.feature)}>
      <div className={styles.featureIcon}>{icon}</div>
      <Heading as="h3">{title}</Heading>
      <p>{description}</p>
    </div>
  );
}

export default function Home(): JSX.Element {
  const {siteConfig} = useDocusaurusContext();
  return (
    <Layout
      title={siteConfig.title}
      description="Build modern infrastructure providers with the power of Rust. Type-safe, high-performance, and secure by default.">
      <HomepageHeader />
      <main>
        <section className={styles.nextPhase}>
          <div className={styles.nextPhaseInner}>
            <div className={styles.nextPhaseMedia} aria-hidden="true">
              <img className={styles.nextPhaseImage} src="img/design.png" alt="" />
            </div>
            <div className={styles.nextPhaseContent}>
              <Heading as="h2" className={styles.nextPhaseTitle}>
                A type-safe IAM SDK
                <br />
                built for the future
              </Heading>
              <p className={styles.nextPhaseLead}>
                iamctl Rust SDK makes building secure IAM providers fast, predictable, and maintainable.
              </p>
              <p className={styles.nextPhaseBody}>
                Use strongly-typed resources and schemas to model your domain, validate inputs, and generate
                consistent plans and applies.
              </p>
              <p className={styles.nextPhaseBody}>
                Built-in JSON-RPC server, state management, and observability let you focus on provider
                logic instead of boilerplate.
              </p>
              <p className={styles.nextPhaseNote}>You’ll never want to go back!</p>
            </div>
          </div>
        </section>
        <section className={styles.howItWorks}>
          <div className="container">
            <Heading as="h2" className={styles.howItWorksTitle}>How it works</Heading>
            <div className="row">
              {HowItWorksList.map(({icon, title, description}, idx) => (
                <HowItWorksStep key={idx} icon={icon} title={title} description={description} />
              ))}
            </div>
          </div>
        </section>
        <section className={styles.features}>
          <div className="container">
            <Heading as="h2" className={styles.featuresTitle}>Core benefits</Heading>
            <div className="row">
              {FeatureList.map(({icon, title, description}, idx) => (
                <Feature key={idx} icon={icon} title={title} description={description} />
              ))}
            </div>
          </div>
        </section>
      </main>
    </Layout>
  );
}
