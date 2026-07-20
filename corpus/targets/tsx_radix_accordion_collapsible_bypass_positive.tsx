// [frensense]
// observation: An Accordion with `collapsible` prop set to `true` allows all panels to be closed simultaneously, which can hide required content (e.g., terms of service, mandatory disclosures, or billing summary) that the user must see before proceeding.
// impact: A user can collapse all accordion panels, unintentionally hiding legally required disclosures, mandatory settings, or critical terms. If the user proceeds without seeing this content, the application may face compliance violations (GDPR, CCPA, etc.) or the user may unknowingly agree to unfavorable terms.
// improvement: Disable `collapsible` when the accordion contains mandatory content, or keep at least one panel always open via the `value` prop.

import * as Accordion from '@radix-ui/react-accordion';
import { ChevronDownIcon } from '@radix-ui/react-icons';

export function TermsAccordion() {
  return (
    <Accordion.Root type="multiple" collapsible>
      <Accordion.Item value="privacy">
        <Accordion.Header>
          <Accordion.Trigger>
            Privacy Policy <ChevronDownIcon />
          </Accordion.Trigger>
        </Accordion.Header>
        <Accordion.Content>Your data is handled per our privacy policy.</Accordion.Content>
      </Accordion.Item>
      <Accordion.Item value="terms">
        <Accordion.Header>
          <Accordion.Trigger>
            Terms of Service <ChevronDownIcon />
          </Accordion.Trigger>
        </Accordion.Header>
        <Accordion.Content>By using this service you agree to the terms.</Accordion.Content>
      </Accordion.Item>
      <Accordion.Item value="billing">
        <Accordion.Header>
          <Accordion.Trigger>
            Billing Summary <ChevronDownIcon />
          </Accordion.Trigger>
        </Accordion.Header>
        <Accordion.Content>You will be charged $49.99/month.</Accordion.Content>
      </Accordion.Item>
    </Accordion.Root>
  );
}
