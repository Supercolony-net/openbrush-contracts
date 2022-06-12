const lightCodeTheme = require('prism-react-renderer/themes/vsLight')
const darkCodeTheme = require('prism-react-renderer/themes/vsDark')

/** @type {import('@docusaurus/types').DocusaurusConfig} */
module.exports = {
  title: 'OpenBrush',
  tagline: 'OpenBrush contracts documentation',
  url: 'https://docs.openbrush.io',
  baseUrl: '/',
  onBrokenLinks: 'throw',
  onBrokenMarkdownLinks: 'warn',
  favicon: 'img/favicon.svg',
  organizationName: 'supercolony-net',
  projectName: 'openbrush-contracts',
  themeConfig: {
    colorMode: {
      defaultMode: 'dark'
    },
    navbar: {
      logo: {
        alt: 'OpenBrush',
        src: 'img/logo.svg',
        srcDark: 'img/logo-dark.svg'
      },
      items: [
        {
          to: 'smart-contracts/overview',
          position: 'right',
          label: 'Examples',
          activeBasePath: 'smart-contracts'
        },
        {
          to: 'deployment',
          position: 'right',
          label: 'Deployment'
        },
        {
          href: 'https://twitter.com/supercolony_net',
          className: 'header-twitter-link',
          position: 'right'
        },
        {
          href: 'https://github.com/Supercolony-net/openbrush-contracts',
          className: 'header-github-link',
          position: 'right'
        }
      ]
    },
    footer: {
      copyright: `Copyright © ${new Date().getFullYear()} OpenBrush, Supercolony.net.`
    },
    prism: {
      theme: lightCodeTheme,
      darkTheme: darkCodeTheme,
      additionalLanguages: ['toml', 'rust']
    }
  },
  plugins: ['docusaurus-plugin-sass'],
  presets: [
    [
      '@docusaurus/preset-classic',
      {
        docs: {
          routeBasePath: '/',
          sidebarPath: require.resolve('./sidebars.js'),
          editUrl: 'https://github.com/Supercolony-net/openbrush-contracts/tree/main/docs'
        },
        theme: {
          customCss: [require.resolve('./src/css/custom.scss')]
        }
      }
    ]
  ]
}
