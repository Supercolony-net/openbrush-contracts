import React from 'react'
import Layout from '@theme/Layout'
import Link from '@docusaurus/Link'
import versions from './../../versions.json'


export default function AllVersions() {
  const current_version = JSON.parse(JSON.stringify(versions[0]))
  let past_versions = JSON.parse(JSON.stringify(versions))
  delete past_versions[0]

  return (
    <Layout>
      <div className="all-versions-header">
        <h1>
          OpenBrush documentation versions
        </h1>
      </div>
      <main className="main-versions-block">
        <div className="versions-block">
          <h3>
            Current version
          </h3>
          <table>
            <tbody>
              <tr>
                <th>{current_version}</th>
                <td>
                  <Link to='/'>
                  Documentation
                  </Link>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
        <div className="versions-block">
          <h3>
            Past versions
          </h3>
          <table>
            <tbody>
              {past_versions.map((version) => (
                <tr key={version}>
                  <th>{version}</th>
                  <td>
                    <Link to={'/'+version}>
                    Documentation
                    </Link>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </main>
    </Layout>
  )
}