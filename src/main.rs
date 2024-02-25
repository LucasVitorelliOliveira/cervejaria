const QTD_POR_CAIXA: i32 = 24;
const ML_GARRAFA: i32 = 600;
const L_TO_ML: f64 = 1000.0;
const PRECO_CAIXA: f64 = 30.0;
fn main () {
    let qtd_caixas = 18;
    let qtd_garrafas = QTD_POR_CAIXA * qtd_caixas;
    let qtd_ml = qtd_garrafas * ML_GARRAFA;
    let qtd_litros = qtd_ml as f64 / L_TO_ML;
    let preco_total = qtd_caixas as f64 * PRECO_CAIXA;

    println!("Foram consumidas {qtd_caixas} caixas; \n\
              Um total de {qtd_garrafas} garrafas; \n\
              Um total de {qtd_litros} litros; \n\
              O preço total foi de R${preco_total:.2}");
}