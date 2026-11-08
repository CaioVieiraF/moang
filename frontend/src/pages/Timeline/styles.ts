import styled from 'styled-components'

export const TimelineList = styled.ul`
  display: flex;
  flex-direction: column;
  gap: 1rem;

  list-style: none;
`

export const TimelineEmpty = styled.p`
  padding: 2rem 0;
  text-align: center;
  color: ${props => props.theme['secondary-300']};

  button {
    border: 0;
    background: transparent;
    color: ${props => props.theme['accent-200']};
    font-weight: bold;
    text-decoration: underline;
    cursor: pointer;
  }
`
