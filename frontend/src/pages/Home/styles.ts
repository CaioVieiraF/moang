import styled from 'styled-components'

export const HomeContainer = styled.main`
  max-width: 76rem;
  height: 100%;
  width: 100%;
  margin: -5rem auto;
  border-radius: 3px;
`
export const HomePostsContainer = styled.div`
  margin-top: 4rem;

  ul {
    list-style-type: none;
  }
`

export const Title = styled.div`
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: space-between;

  margin: 2rem auto;

  small {
    color: ${props => props.theme['text-300']};
  }
`
