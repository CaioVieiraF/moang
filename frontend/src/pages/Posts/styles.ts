import styled from 'styled-components'

export const PostsContainer = styled.main`
  max-width: 76rem;
  height: 100%;
  width: 100%;

  margin: 2rem auto;
  border-radius: 3px;

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
