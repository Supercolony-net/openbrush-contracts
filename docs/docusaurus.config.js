const lightCodeTheme = require('prism-react-renderer/themes/vsLight')
const darkCodeTheme = require('prism-react-renderer/themes/vsDark')
const versions = require('./versions.json')

/** @type {import('@docusaurus/types').DocusaurusConfig} */
module.exports = {
  title: 'OpenBrush',
  tagline: 'OpenBrush contracts documentation',
  url: 'https://docs.openbrush.test.io',
  baseUrl: '/docs/',
  onBrokenLinks: 'throw',
  onBrokenMarkdownLinks: 'warn',
  favicon: 'img/favicon.svg',
  organizationName: 'o-tsaruk',
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
          type: 'dropdown',
          label: 'versions',
          position: 'left',
          items: [
            {to: '/'+versions[1], label: versions[1]},
            {to: '/'+versions[2], label: versions[2]},
            {to: 'allVersions', label: 'all versions'}]
        },
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
          editUrl: 'https://github.com/Supercolony-net/openbrush-contracts/tree/main/docs',
          includeCurrentVersion: false
        },
        theme: {
          customCss: [require.resolve('./src/css/custom.scss')]
        }
      }
    ]
  ]
}
