import styled from 'styled-components'

export const BannerContainer = styled.div`
  max-width: 76rem;
  height: 100%;
  width: 100%;
  border-radius: 3px;
  border: 1px solid ${props => props.theme['primary-800']};
  background: ${props => props.theme['secondary-800']};
  padding: 3rem;

  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 1rem;

  a {
    text-decoration: none;
    color: ${props => props.theme['accent']};
  }
`
