const FEATURES = [
  'Passive collection',
  'ZK privacy',
  'Economic incentives',
  'Anti-spoofing + slashing',
  'Agent-to-agent mesh',
  'On-chain reputation',
  'Soulbound credentials',
  'No special hardware',
]

type Status = 'yes' | 'no' | 'partial'

const PROJECTS: {
  name: string
  highlight?: boolean
  features: Status[]
}[] = [
  {
    name: 'ZK-PoX',
    highlight: true,
    features: ['yes', 'yes', 'yes', 'yes', 'yes', 'yes', 'yes', 'yes'],
  },
  {
    name: 'Worldcoin',
    features: ['no', 'no', 'no', 'partial', 'no', 'no', 'yes', 'no'],
  },
  {
    name: 'POAP',
    features: ['no', 'no', 'no', 'no', 'no', 'partial', 'yes', 'yes'],
  },
  {
    name: 'zkLocus',
    features: ['no', 'yes', 'no', 'no', 'no', 'no', 'partial', 'yes'],
  },
  {
    name: 'Helium',
    features: ['yes', 'no', 'yes', 'partial', 'no', 'no', 'no', 'no'],
  },
]

function StatusCell({ status }: { status: Status }) {
  if (status === 'yes')
    return <span className="text-accent font-bold">&#x2713;</span>
  if (status === 'no')
    return <span className="text-red/60">&#x2715;</span>
  return <span className="text-dim">~</span>
}

export default function Comparison() {
  return (
    <section className="py-24 border-b border-border">
      <div className="max-w-5xl mx-auto px-6">
        <div className="text-xs tracking-[3px] uppercase text-dim mb-8">
          {'// competitive landscape'}
        </div>
        <h2 className="text-3xl font-bold text-bright mb-4">
          Only system with all eight.
        </h2>
        <p className="text-text mb-12 max-w-xl">
          Passive collection + ZK privacy + economic stakes + agent mesh +
          reputation + soulbound credentials. No other project combines them.
        </p>

        <div className="overflow-x-auto">
          <table className="w-full text-sm border-collapse">
            <thead>
              <tr className="border-b border-border">
                <th className="text-left py-3 pr-4 text-dim font-normal text-xs">
                  Feature
                </th>
                {PROJECTS.map((p) => (
                  <th
                    key={p.name}
                    className={`py-3 px-3 text-center font-medium text-xs ${
                      p.highlight ? 'text-accent' : 'text-dim'
                    }`}
                  >
                    {p.name}
                  </th>
                ))}
              </tr>
            </thead>
            <tbody>
              {FEATURES.map((feat, i) => (
                <tr key={feat} className="border-b border-border/50">
                  <td className="py-3 pr-4 text-text text-xs">{feat}</td>
                  {PROJECTS.map((p) => (
                    <td key={p.name} className="py-3 px-3 text-center">
                      <StatusCell status={p.features[i]} />
                    </td>
                  ))}
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>
    </section>
  )
}
