import { Link } from 'react-router-dom'
import { BannerContainer } from './styles'

export function Banner() {
  return (
    <BannerContainer>
      <h1>Bem vindo ao meu blog!</h1>
      <p>Olá mundo, esse é o meu registro na internet.
        Aqui você encontrará postagens sobre devenvolvimento,
        redes de computadores, algumas estórias e histórias
        aleatórias, desenvolvimento de jogos e outras coisas
        sobre tecnologia.
      </p>
      <strong>Você pode conhecer melhor minha história <Link to="/about">AQUI</Link></strong>
    </BannerContainer>
  )
}
