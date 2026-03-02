import { useState } from 'react'

const STORIES = [
  {
    name: 'Maria',
    role: 'tenant',
    scenario: 'Prove residency for visa renewal',
    conversation: [
      {
        who: 'Maria',
        text: '"Prove to immigration that I\'ve lived at my address for the past 6 months"',
      },
      {
        who: 'Agent',
        text: '"I have 4,320 signed GPS points showing you within 200m of [home] between 10pm-7am on 175 out of 180 nights. Generating ZK proof..."',
      },
      {
        who: 'Agent',
        text: '"Proof generated (2.1 KB). It proves: \'Device was within 200m of [redacted coordinates] for 175+ nights over 180-day period.\' No exact address revealed. Submit to mesh?"',
      },
    ],
  },
  {
    name: 'Jakub',
    role: 'freelancer',
    scenario: 'Prove work history without employer reference',
    conversation: [
      {
        who: 'Jakub',
        text: '"Generate a proof that I commuted to a workplace 5 days/week for 2 years"',
      },
      {
        who: 'Agent',
        text: '"Analyzing GPS trajectory... Found consistent A→B pattern, Monday-Friday, for 480+ days across 24 months. Generating ZK proof..."',
      },
      {
        who: 'Agent',
        text: '"Proof ready. Regular 5-day/week commute between two consistent locations for 24 months. No addresses or routes revealed."',
      },
    ],
  },
  {
    name: 'Anna',
    role: 'driver',
    scenario: 'Lower insurance premiums',
    conversation: [
      {
        who: 'Anna',
        text: '"Show my insurer I have low-risk driving patterns"',
      },
      {
        who: 'Agent',
        text: '"Analyzing 12-month GPS data... Home by 10pm 90% of nights, no high-risk zones, avg 12km daily range. Generating STABILITY_PROOF..."',
      },
      {
        who: 'Agent',
        text: '"Insurer\'s agent offers 15% premium reduction for verified low-risk credential. Accept? Fee: 1.50 USDC via escrow."',
      },
    ],
  },
]

export default function UserStory() {
  const [active, setActive] = useState(0)
  const story = STORIES[active]

  return (
    <section className="py-24 border-b border-border">
      <div className="max-w-5xl mx-auto px-6">
        <div className="text-xs tracking-[3px] uppercase text-dim mb-8">
          {'// user stories'}
        </div>
        <h2 className="text-3xl font-bold text-bright mb-4">
          Talk to your agent. Get proof.
        </h2>
        <p className="text-text mb-12 max-w-xl">
          Users interact with their ZeroClaw agent in natural language.
          The agent handles proof generation, delivery, and payment.
        </p>

        <div className="flex gap-3 mb-8 overflow-x-auto pb-2">
          {STORIES.map((s, i) => (
            <button
              key={s.name}
              onClick={() => setActive(i)}
              className={`shrink-0 px-4 py-2 rounded-md text-xs font-medium transition-all ${
                active === i
                  ? 'bg-purple text-white'
                  : 'bg-surface border border-border text-dim hover:text-text hover:border-purple/30'
              }`}
            >
              {s.name} ({s.role})
            </button>
          ))}
        </div>

        <div className="bg-surface border border-border rounded-lg overflow-hidden">
          <div className="px-5 py-3 border-b border-border flex items-center justify-between">
            <span className="text-xs text-dim">{story.scenario}</span>
            <span className="text-xs text-accent">ZeroClaw</span>
          </div>
          <div className="p-5 space-y-4">
            {story.conversation.map((msg, i) => (
              <div
                key={i}
                className={`flex gap-3 ${
                  msg.who !== 'Agent' ? 'justify-end' : ''
                }`}
              >
                {msg.who === 'Agent' && (
                  <span className="shrink-0 w-8 h-8 rounded-full bg-purple/20 text-purple text-xs flex items-center justify-center font-bold">
                    AI
                  </span>
                )}
                <div
                  className={`max-w-md rounded-lg px-4 py-3 text-sm leading-relaxed ${
                    msg.who === 'Agent'
                      ? 'bg-surface-2 text-text border border-border'
                      : 'bg-purple/10 text-bright border border-purple/20'
                  }`}
                >
                  {msg.text}
                </div>
              </div>
            ))}
          </div>
        </div>
      </div>
    </section>
  )
}
