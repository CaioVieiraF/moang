import styled from 'styled-components'

export const BannerContainer = styled.section`
  display: flex;
  flex-direction: column;
  gap: 0.85rem;

  padding: 2rem 2.25rem;
  border-radius: 3px;
  border: 1px solid ${props => props.theme['secondary-400']}70;
  border-left: 3px solid ${props => props.theme['accent-400']};
  background: linear-gradient(
    to bottom right,
    ${props => props.theme['secondary-900']},
    ${props => props.theme['secondary-950']}
  );
  box-shadow: 0 25px 45px rgba(0, 0, 0, .2);

  h1 {
    font-size: 1.75rem;
    color: ${props => props.theme['accent-100']};
  }

  p {
    max-width: 62ch;
    line-height: 1.7;
    color: ${props => props.theme['text-200']};
  }

  a {
    align-self: flex-start;
    font-weight: bold;
    text-decoration: none;
    color: ${props => props.theme['accent-200']};
  }

  a:hover {
    text-decoration: underline;
  }

  @media (max-width: 600px) {
    padding: 1.5rem 1.25rem;

    h1 {
      font-size: 1.45rem;
    }
  }
`
